using Server.DataTransferObjects;
using Server.Models;
using Server.Repository;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

builder.Services.AddControllers();
builder.Services.AddScoped<IBlogPostRepository, BlogPostRepository>();
builder.Services.AddAutoMapper(cfg => 
{
    cfg.CreateMap<BlogPost, BlogPostDto>();
});

WebApplication app = builder.Build();

app.MapControllers();

app.Run();
