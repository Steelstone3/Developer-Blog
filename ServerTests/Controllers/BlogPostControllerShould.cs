using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using Moq;
using Server.Controllers;
using Server.DataTransferObjects;

namespace ServerTests.Controllers
{
    public class BlogPostControllerShould
    {
        private readonly Mock<IMapper> mockMapper;
        private readonly BlogPostController controller;

        public BlogPostControllerShould()
        {
            mockMapper = new Mock<IMapper>();
            controller = new BlogPostController(mockMapper.Object);
        }

        [Fact]
        public void GetBlog200()
        {
            // Given
            int id = 1;
            BlogPostDto expectedDto = new()
            {
                Id = id,
                Title = "Example Title",
                Content = "Example Content"
            };
            mockMapper.Setup(m => m.Map<BlogPostDto>(It.IsAny<object>()))
                       .Returns(expectedDto);

            // When
            ActionResult<BlogPostDto> result = controller.GetBlog(id);

            // Then
            OkObjectResult okResult = Assert.IsType<OkObjectResult>(result.Result);
            Assert.Equal(200, okResult.StatusCode);
            BlogPostDto returnedDto = Assert.IsType<BlogPostDto>(okResult.Value);
            Assert.Equal(expectedDto.Id, returnedDto.Id);
            Assert.Equal(expectedDto.Title, returnedDto.Title);
            Assert.Equal(expectedDto.Content, returnedDto.Content);
        }
    }
}