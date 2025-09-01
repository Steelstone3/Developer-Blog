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

        [HttpGet("{id}")]
        public ActionResult<BlogPostDto> GetBlogPost(int id)
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
    }
}