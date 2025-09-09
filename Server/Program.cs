using Server.DataTransferObjects;
using Server.Models;
using Server.Repository;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.Services.AddControllers();
builder.Services.AddSingleton<IBlogPostRepository>(provider => new BlogPostRepository(new Seeding("blog_posts.json")));
builder.Services.AddAutoMapper(cfg =>
{
    cfg.CreateMap<BlogPost, BlogPostDto>();
});

WebApplication app = builder.Build();

app.MapControllers();

app.Run();
