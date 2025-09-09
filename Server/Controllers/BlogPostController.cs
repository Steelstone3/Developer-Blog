using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using Server.DataTransferObjects;
using Server.Models;
using Server.Repository;

namespace Server.Controllers
{
    [ApiController]
    [Route("blogs")]
    public class BlogPostController : ControllerBase
    {
        private readonly IBlogPostRepository blogPostRepository;
        private readonly IMapper mapper;

        public BlogPostController(IBlogPostRepository blogPostRepository, IMapper mapper)
        {
            this.blogPostRepository = blogPostRepository;
            this.mapper = mapper;
        }

        [HttpGet("all")]
        public ActionResult<BlogPostDto> GetAllBlogPost()
        {
            if (blogPostRepository.BlogPosts is null || blogPostRepository.BlogPosts.Count == 0)
            {
                return NotFound();
            }

            List<BlogPostDto> blogPostsDto = mapper.Map<List<BlogPostDto>>(blogPostRepository.BlogPosts);

            return Ok(blogPostsDto);
        }

        [HttpGet]
        public ActionResult<BlogPostDto> GetBlogPost([FromQuery] int id)
        {
            BlogPost blogPost = blogPostRepository.GetById(id);

            if (blogPost is null)
            {
                return NotFound();
            }

            BlogPostDto blogPostDto = mapper.Map<BlogPostDto>(blogPost);

            return Ok(blogPostDto);
        }

        [HttpPost]
        public ActionResult<BlogPostDto> PostBlogPost([FromBody] BlogPost blogPost)
        {
            bool isSuccess = blogPostRepository.AddBlog(blogPost);

            if (!isSuccess)
            {
                return Conflict();
            }

            BlogPostDto blogPostDto = mapper.Map<BlogPostDto>(blogPost);

            return Created(string.Empty, blogPostDto);
        }

        [HttpDelete]
        public ActionResult<BlogPostDto> DeleteBlogPost([FromQuery] int id)
        {
            bool isSuccess = blogPostRepository.DeleteBlogById(id);

            if (!isSuccess)
            {
                return NotFound();
            }

            return NoContent();
        }

        [HttpPut]
        public ActionResult<BlogPostDto> UpdateBlogPost([FromBody] BlogPost blogPost)
        {
            bool isSuccess = blogPostRepository.UpdateBlog(blogPost);

            if (!isSuccess)
            {
                return NotFound();
            }

            BlogPostDto blogPostDto = mapper.Map<BlogPostDto>(blogPost);

            return Created(string.Empty, blogPostDto);
        }
    }
}