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
        public ActionResult<BlogPostDto> GetBlog(int id)
        {
            BlogPost blog = blogPostRepository.GetById(id);
            BlogPostDto blogPostDto = mapper.Map<BlogPostDto>(blog);

            return Ok(blogPostDto);
        }
    }
}