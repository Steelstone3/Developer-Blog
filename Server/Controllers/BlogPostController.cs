using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using Server.DataTransferObjects;

namespace Server.Controllers
{
    [ApiController]
    [Route("blogs")]
    public class BlogPostController : ControllerBase
    {
        private readonly IMapper _mapper;

        public BlogPostController(IMapper mapper)
        {
            _mapper = mapper;
        }

        [HttpGet("{id}")]
        public ActionResult<BlogPostDto> GetBlog(int id)
        {
            // TODO AH from a data store
            BlogPostDto blog = new BlogPostDto
            {
                Id = id,
                Name = "Example Product",
                Price = 19.99m
            };

            BlogPostDto blogPostDto = _mapper.Map<BlogPostDto>(blog);

            return Ok(blogPostDto);
        }
    }
}