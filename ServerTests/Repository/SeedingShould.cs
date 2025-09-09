public class SeedingShould : IDisposable
{
    private readonly Seeding seeding = new();
    private const string TestFilePath = "seeding_test_file.json";

    public void Dispose()
    {
        File.Delete(TestFilePath);
    }

    [Fact]
    public void SeedBlogPostsFile()
    {
        // When
        seeding.Seed(TestFilePath);

        // Then
        Assert.True(File.Exists(TestFilePath));
    }
}