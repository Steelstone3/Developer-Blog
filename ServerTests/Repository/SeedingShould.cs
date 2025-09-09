using Server.Repository;

namespace ServerTests.Repository
{
    public class SeedingShould : IDisposable
    {
        private Seeding seeding;

        public SeedingShould()
        {
            seeding = new(TestFilePath);
        }

        private const string TestFilePath = "seeding_test_file.json";

        [Fact]
        public void SeedBlogPostsFile()
        {
            // When
            seeding.Seed();

            // Then
            Assert.True(File.Exists(TestFilePath));
        }

        [Fact]
        public void SeedBlogPostsNoFileNameProvided()
        {
            // Given
            seeding = new(string.Empty);

            // Then
            Assert.Throws<Exception>(() => seeding.Seed());
        }

        public void Dispose()
        {
            File.Delete(TestFilePath);
        }
    }
}