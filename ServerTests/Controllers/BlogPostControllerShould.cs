using System.Net;
using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using Moq;
using Server.Controllers;
using Server.DataTransferObjects;

namespace ServerTests.Controllers
{
    public class BlogPostControllerShould
    {
        private readonly Mock<IMapper> _mockMapper;
        private readonly BlogPostController _controller;

        public BlogPostControllerShould()
        {
            _mockMapper = new Mock<IMapper>();
            _controller = new BlogPostController(_mockMapper.Object);
        }

        [Fact]
        public void GetBlog200()
        {
            // Given
            int testId = 1;
            BlogPostDto expectedDto = new()
            {
                Id = testId,
                Title = "Example Title",
                Content = "Example Content"
            };
            _mockMapper.Setup(m => m.Map<BlogPostDto>(It.IsAny<object>()))
                       .Returns(expectedDto);

            // When
            ActionResult<BlogPostDto> result = _controller.GetBlog(testId);

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