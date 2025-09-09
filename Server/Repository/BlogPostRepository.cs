using System.Text.Json;
using Server.Models;

namespace Server.Repository
{
    public class BlogPostRepository : IBlogPostRepository
    {
        public List<BlogPost> BlogPosts { get; private set; }

        public BlogPostRepository(string filePath)
        {
            new Seeding().Seed(filePath);
            string json = File.ReadAllText(filePath);
            BlogPosts = JsonSerializer.Deserialize<List<BlogPost>>(json);
        }

        public bool AddBlog(BlogPost blogPost)
        {
            if (IsUniqueById(blogPost.Id))
            {
                return false;
            }

            BlogPosts.Add(blogPost);

            return true;
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

        public void UpdateById(BlogPost blogPost)
        {
            if (!IsUniqueById(blogPost.Id))
            {
                return;
            }

            DeleteBlogById(blogPost.Id);
            AddBlog(blogPost);
        }

        private bool IsUniqueById(int id) => BlogPosts.Select(bp => bp.Id).Contains(id);
    }

    public interface IBlogPostRepository
    {
        List<BlogPost> BlogPosts { get; }
        BlogPost GetById(int id);
        bool AddBlog(BlogPost blogPost);
        void DeleteBlogById(int id);
        void UpdateById(BlogPost blogPost);
    }
}