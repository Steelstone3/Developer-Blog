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
            if (IsUniqueById(blogPost.Id))
            {
                return;
            }

            BlogPosts.Add(blogPost);
        }

        public void DeleteBlogById(int id)
        {
            if (!IsUniqueById(id))
            {
                return;
            }

            BlogPost blogPost = GetById(id);
            BlogPosts.Remove(blogPost);
        }

        public BlogPost GetById(int id) => BlogPosts.FirstOrDefault(b => b.Id == id);

        public void UpdateById(int id, BlogPost blogPost)
        {
            if (id != blogPost.Id || !IsUniqueById(id))
            {
                return;
            }

            DeleteBlogById(id);
            AddBlog(blogPost);
        }

        private bool IsUniqueById(int id) => BlogPosts.Select(bp => bp.Id).Contains(id);
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