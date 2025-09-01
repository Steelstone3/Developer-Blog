using Server.Models;
using Server.Repository;

namespace ServerTests.Repository
{
    public class BlogPostRepositoryShould
    {
        IBlogPostRepository blogPostRepository = new BlogPostRepository();

        [Fact]
        public void GetBlogByIdNotFound()
        {
            // When
            BlogPost blog = blogPostRepository.GetById(696969);

            // Then
            Assert.Null(blog);
        }

        [Theory]
        [InlineData(0, "I like blogs", "This blog was brought to you by people who like blogs.", "Harold", "Harold@hello.com", false)]
        [InlineData(1, "No I REALLY like blogs", "This blog writter is serious!", "Jeff", "Jeff@hello.com", true)]
        public void GetBlogById(int id, string title, string content, string authorId, string authorEmail, bool isPublished)
        {
            // Given
            BlogPost expectedBlogPost = new(id, title, content, authorId, authorEmail, isPublished);

            // When
            BlogPost blog = blogPostRepository.GetById(id);

            // Then
            Assert.Equivalent(expectedBlogPost, blog);
        }

        [Theory]
        [InlineData(0, "New Blog", "This is a new blog.", "Harold", "Harold@hello.com", true)]
        [InlineData(1, "Blogger Person", "What is it like to be a blogger person?", "Jeff", "Jeff@hello.com", false)]
        public void AddBlogRequiresUniqueId(int id, string title, string content, string authorId, string authorEmail, bool isPublished)
        {
            // Given
            BlogPost blogPost = new(id, title, content, authorId, authorEmail, isPublished);

            // When
            blogPostRepository.AddBlog(blogPost);

            // Then
            Assert.NotEmpty(blogPostRepository.BlogPosts);
            Assert.Equal(2, blogPostRepository.BlogPosts.Count);
        }

        [Theory]
        [InlineData(2, "New Blog", "This is a new blog.", "Harold", "Harold@hello.com", true)]
        [InlineData(3, "Blogger Person", "What is it like to be a blogger person?", "Jeff", "Jeff@hello.com", false)]
        public void AddBlog(int id, string title, string content, string authorId, string authorEmail, bool isPublished)
        {
            // Given
            BlogPost blogPost = new(id, title, content, authorId, authorEmail, isPublished);

            // When
            blogPostRepository.AddBlog(blogPost);

            // Then
            Assert.NotEmpty(blogPostRepository.BlogPosts);
            Assert.Equivalent(blogPostRepository.BlogPosts.LastOrDefault(), blogPost);
        }
    }
}