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
        public void GetAllBlogs200()
        {
            // Given
            List<BlogPost> expectedModels = [new(1, "Title", "Content", "Author Id", "Author Email", false)];
            List<BlogPostDto> expectedDtos = [new(1, "Title", "Content")];

            blogPostRepository.Setup(bpr => bpr.BlogPosts).Returns(expectedModels);
            mockMapper.Setup(m => m.Map<List<BlogPostDto>>(It.IsAny<object>()))
                       .Returns(expectedDtos);

            // When
            ActionResult<BlogPostDto> result = controller.GetAllBlogPost();

            // Then
            blogPostRepository.VerifyAll();
            mockMapper.VerifyAll();
            OkObjectResult okResult = Assert.IsType<OkObjectResult>(result.Result);
            List<BlogPostDto> returnedDtos = Assert.IsType<List<BlogPostDto>>(okResult.Value);
            Assert.Equal(expectedDtos[0].Id, returnedDtos[0].Id);
            Assert.Equal(expectedDtos[0].Title, returnedDtos[0].Title);
            Assert.Equal(expectedDtos[0].Content, returnedDtos[0].Content);
        }

        [Fact]
        public void GetAllBlogs404NullBlogPosts()
        {
            // Given
            blogPostRepository.Setup(repo => repo.BlogPosts).Returns((List<BlogPost>)null);

            // When
            ActionResult<BlogPostDto> result = controller.GetAllBlogPost();

            // Then
            blogPostRepository.VerifyAll();
            Assert.IsType<NotFoundResult>(result.Result);
        }

        [Fact]
        public void GetAllBlogs404EmptyBlogPosts()
        {
            // Given
            blogPostRepository.Setup(repo => repo.BlogPosts).Returns((List<BlogPost>)new());

            // When
            ActionResult<BlogPostDto> result = controller.GetAllBlogPost();

            // Then
            blogPostRepository.VerifyAll();
            Assert.IsType<NotFoundResult>(result.Result);
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
            blogPostRepository.VerifyAll();
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
            OkObjectResult ok = Assert.IsType<OkObjectResult>(result.Result);
            BlogPostDto returnedDto = Assert.IsType<BlogPostDto>(ok.Value);
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
            Assert.IsType<ConflictResult>(result.Result);
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
            CreatedResult created = Assert.IsType<CreatedResult>(result.Result);
            BlogPostDto returnedDto = Assert.IsType<BlogPostDto>(created.Value);
            Assert.Equal(expectedDto.Id, returnedDto.Id);
            Assert.Equal(expectedDto.Title, returnedDto.Title);
            Assert.Equal(expectedDto.Content, returnedDto.Content);
        }

        [Fact]
        public void DeleteBlog404()
        {
            // Given
            int id = 5;
            blogPostRepository.Setup(bpr => bpr.DeleteBlogById(id)).Returns(false);

            // When
            ActionResult<BlogPostDto> result = controller.DeleteBlogPost(id);

            // Then
            blogPostRepository.VerifyAll();
            Assert.IsType<NotFoundResult>(result.Result);
        }

        [Fact]
        public void DeleteBlog204()
        {
            // Given
            int id = 0;
            blogPostRepository.Setup(bpr => bpr.DeleteBlogById(id)).Returns(true);

            // When
            ActionResult<BlogPostDto> result = controller.DeleteBlogPost(id);

            // Then
            blogPostRepository.VerifyAll();
            Assert.IsType<NoContentResult>(result.Result);
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