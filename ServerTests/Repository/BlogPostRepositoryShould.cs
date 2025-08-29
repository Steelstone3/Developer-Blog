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

        [Fact]
        public void GetBlogById()
        {
            // Given
            BlogPost expectedBlogPost = new()
            {
                Id = 0,
                Title = "I like blogs",
                Content = "This blog was brought to you by people who like blogs."
            };

            // When
            BlogPost blog = blogPostRepository.GetById(0);

            // Then
            Assert.Equivalent(expectedBlogPost, blog);
        }
    }
}