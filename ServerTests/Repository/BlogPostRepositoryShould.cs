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
        [InlineData(0, "I like blogs", "This blog was brought to you by people who like blogs.")]
        [InlineData(1, "No I REALLY like blogs", "This blog writter is serious!")]
        public void GetBlogById(int id, string title, string content)
        {
            // Given
            BlogPost expectedBlogPost = new()
            {
                Id = id,
                Title = title,
                Content = content
            };

            // When
            BlogPost blog = blogPostRepository.GetById(id);

            // Then
            Assert.Equivalent(expectedBlogPost, blog);
        }
    }
}