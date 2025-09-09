using System.Text.Json;
using Server.Models;
using static Server.Repository.Seeding;

namespace Server.Repository
{
    public class BlogPostRepository : IBlogPostRepository
    {
        private readonly ISeeding seeding;

        public List<BlogPost> BlogPosts { get; private set; }

        public BlogPostRepository(ISeeding seeding)
        {
            this.seeding = seeding;
            seeding.Seed();
            string json = File.ReadAllText(seeding.FilePath);
            BlogPosts = JsonSerializer.Deserialize<List<BlogPost>>(json);
        }

        public bool AddBlog(BlogPost blogPost)
        {
            if (IsUniqueById(blogPost.Id))
            {
                return false;
            }

            BlogPosts.Add(blogPost);
            SerializeBlogPosts();

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

            SerializeBlogPosts();
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

        private void SerializeBlogPosts()
        {
            string jsonString = JsonSerializer.Serialize(BlogPosts, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(seeding.FilePath, jsonString);
        }
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