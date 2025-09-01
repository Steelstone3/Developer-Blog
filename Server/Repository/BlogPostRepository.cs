using System.Text.Json;
using Server.Models;

namespace Server.Repository
{
    public class BlogPostRepository : IBlogPostRepository
    {
        private readonly List<BlogPost> blogPosts;

        public BlogPostRepository()
        {
            string filePath = Path.Combine(Directory.GetCurrentDirectory(), "Repository/blog_posts.json");
            string json = File.ReadAllText(filePath);
            blogPosts = JsonSerializer.Deserialize<List<BlogPost>>(json);
        }

        public BlogPost GetById(int id)
        {
            return blogPosts.FirstOrDefault(b => b.Id == id);
        }
    }

    public interface IBlogPostRepository
    {
        BlogPost GetById(int id);
    }
}