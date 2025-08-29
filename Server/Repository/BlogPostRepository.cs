using System.Text.Json;
using Server.Models;

namespace Server.Repository
{
    public class BlogPostRepository : IBlogPostRepository
    {
        private readonly List<BlogPost> _blogPosts;

        public BlogPostRepository()
        {
            // Get the full path to the JSON file
            string filePath = Path.Combine(Directory.GetCurrentDirectory(), "blog_posts.json");
            string json = File.ReadAllText(filePath);
            _blogPosts = JsonSerializer.Deserialize<List<BlogPost>>(json);
        }

        public BlogPost GetById(int id)
        {
            return _blogPosts.FirstOrDefault(b => b.Id == id);
        }
    }

    public interface IBlogPostRepository
    {
        BlogPost GetById(int id);
    }
}