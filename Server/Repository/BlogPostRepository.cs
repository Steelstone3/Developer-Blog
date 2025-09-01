using System.Text.Json;
using Server.Models;

namespace Server.Repository
{
    public class BlogPostRepository : IBlogPostRepository
    {
        public List<BlogPost> BlogPosts { get; private set; }

        public BlogPostRepository()
        {
            string filePath = Path.Combine(Directory.GetCurrentDirectory(), "Repository/blog_posts.json");
            string json = File.ReadAllText(filePath);
            BlogPosts = JsonSerializer.Deserialize<List<BlogPost>>(json);
        }

        public void AddBlog(BlogPost blogPost)
        {
            BlogPosts.Add(blogPost);
        }

        public void DeleteBlogById(int id)
        {
            throw new NotImplementedException();
        }

        public BlogPost GetById(int id)
        {
            return BlogPosts.FirstOrDefault(b => b.Id == id);
        }

        public void UpdateById(int id, BlogPost blogPost)
        {
            throw new NotImplementedException();
        }
    }

    public interface IBlogPostRepository
    {
        List<BlogPost> BlogPosts { get; }
        BlogPost GetById(int id);
        void AddBlog(BlogPost blogPost);
        void DeleteBlogById(int id);
        void UpdateById(int id, BlogPost blogPost);
    }
}