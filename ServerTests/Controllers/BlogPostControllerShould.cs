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
        public void GetBlog404()
        {
            // Given
            int id = 99;
            blogPostRepository.Setup(repo => repo.GetById(id)).Returns((BlogPost)null);

            // When
            ActionResult<BlogPostDto> result = controller.GetBlogPost(id);

            // Then
            Assert.IsType<NotFoundResult>(result.Result);
        }

        [Fact]
        public void GetBlog200()
        {
            // Given
            int id = 1;

            BlogPost expectedModel = new(id, "Title", "Content", "Author Id", "Author Email", false);
            BlogPostDto expectedDto = new(id, "Title", "Content");

            blogPostRepository.Setup(bpr => bpr.GetById(id)).Returns(expectedModel);
            mockMapper.Setup(m => m.Map<BlogPostDto>(It.IsAny<object>()))
                       .Returns(expectedDto);

            // When
            ActionResult<BlogPostDto> result = controller.GetBlogPost(id);

            // Then
            blogPostRepository.VerifyAll();
            mockMapper.VerifyAll();
            OkObjectResult okResult = Assert.IsType<OkObjectResult>(result.Result);
            Assert.Equal(200, okResult.StatusCode);
            BlogPostDto returnedDto = Assert.IsType<BlogPostDto>(okResult.Value);
            Assert.Equal(expectedDto.Id, returnedDto.Id);
            Assert.Equal(expectedDto.Title, returnedDto.Title);
            Assert.Equal(expectedDto.Content, returnedDto.Content);
        }

        [Fact]
        public void PostBlog409()
        {
            // Given
            int id = 0;

            BlogPost expectedModel = new(id, "Title", "Content", "Author Id", "Author Email", false);
            BlogPostDto expectedDto = new(id, "Title", "Content");

            blogPostRepository.Setup(bpr => bpr.AddBlog(expectedModel)).Returns(false);

            // When
            ActionResult<BlogPostDto> result = controller.PostBlogPost(expectedModel);

            // Then
            blogPostRepository.VerifyAll();
            ConflictResult conflictResult = Assert.IsType<ConflictResult>(result.Result);
            Assert.Equal(409, conflictResult.StatusCode);
        }

        [Fact]
        public void PostBlog201()
        {
            // Given
            int id = 99;

            BlogPost expectedModel = new(id, "Title", "Content", "Author Id", "Author Email", false);
            BlogPostDto expectedDto = new(id, "Title", "Content");

            blogPostRepository.Setup(bpr => bpr.AddBlog(expectedModel)).Returns(true);
            mockMapper.Setup(m => m.Map<BlogPostDto>(It.IsAny<object>()))
                       .Returns(expectedDto);

            // When
            ActionResult<BlogPostDto> result = controller.PostBlogPost(expectedModel);

            // Then
            blogPostRepository.VerifyAll();
            mockMapper.VerifyAll();
            CreatedResult createdResult = Assert.IsType<CreatedResult>(result.Result);
            Assert.Equal(201, createdResult.StatusCode);
            BlogPostDto returnedDto = Assert.IsType<BlogPostDto>(createdResult.Value);
            Assert.Equal(expectedDto.Id, returnedDto.Id);
            Assert.Equal(expectedDto.Title, returnedDto.Title);
            Assert.Equal(expectedDto.Content, returnedDto.Content);
        }

        [Fact(Skip = "Todo")]
        public void DeleteBlog200()
        {
            // Given

            // When

            // Then
        }

        [Fact(Skip = "Todo")]
        public void PatchBlog200()
        {
            // Given

            // When

            // Then
        }
    }
}