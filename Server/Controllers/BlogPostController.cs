using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using Server.DataTransferObjects;
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
        public ActionResult<BlogPostDto> GetBlog(int id)
        {
            // TODO AH from a data store
            BlogPostDto blog = new(id);

            BlogPostDto blogPostDto = mapper.Map<BlogPostDto>(blog);

            return Ok(blogPostDto);
        }
    }
}