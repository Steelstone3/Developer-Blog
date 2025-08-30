using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using Moq;
using Server.Controllers;
using Server.DataTransferObjects;
using Server.Models;
using Server.Repository;

namespace ServerTests.Controllers
{
    public class BlogPostControllerShould
    {
        private readonly Mock<IBlogPostRepository> blogPostRepository;
        private readonly Mock<IMapper> mockMapper;
        private readonly BlogPostController controller;

        public BlogPostControllerShould()
        {
            blogPostRepository = new Mock<IBlogPostRepository>();
            mockMapper = new Mock<IMapper>();
            controller = new BlogPostController(blogPostRepository.Object, mockMapper.Object);
        }

        [Fact]
        public void GetBlog200()
        {
            // Given
            int id = 1;

            BlogPost expectedModel = new(id, "", "", "", "", false);
            BlogPostDto expectedDto = new(id);

            blogPostRepository.Setup(bpr => bpr.GetById(id)).Returns(expectedModel);
            mockMapper.Setup(m => m.Map<BlogPostDto>(It.IsAny<object>()))
                       .Returns(expectedDto);

            // When
            ActionResult<BlogPostDto> result = controller.GetBlog(id);

            // Then
            blogPostRepository.VerifyAll();
            mockMapper.VerifyAll();
            OkObjectResult okResult = Assert.IsType<OkObjectResult>(result.Result);
            Assert.Equal(200, okResult.StatusCode);
            BlogPostDto returnedDto = Assert.IsType<BlogPostDto>(okResult.Value);
            Assert.Equal(expectedDto.Id, returnedDto.Id);
        }
    }
}