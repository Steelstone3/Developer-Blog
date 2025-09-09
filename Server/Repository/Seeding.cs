using System.Text.Json;
using Server.Models;
using static Server.Repository.Seeding;

namespace Server.Repository
{
    public class Seeding : ISeeding
    {
        public Seeding(string filePath)
        {
            FilePath = filePath;
        }

        public string FilePath { get; private set; }

        public void Seed()
        {
            if (string.IsNullOrWhiteSpace(FilePath))
            {
                throw new Exception();
            }

            List<BlogPost> blogPosts =
            [
                new BlogPost(
                id: 0,
                title: "I like blogs",
                content: "This blog was brought to you by people who like blogs.",
                authorId: "Harold",
                authorEmail: "Harold@hello.com",
                isPublished: false
            ),
            new BlogPost(
                id: 1,
                title: "No I REALLY like blogs",
                content: "This blog writter is serious!",
                authorId: "Jeff",
                authorEmail: "Jeff@hello.com",
                isPublished: true
            ),
            new BlogPost(
                id: 2,
                title: "Something Title",
                content: "This blogger has no idea what they are doing",
                authorId: "Person",
                authorEmail: "Person@hello.com",
                isPublished: false
            )
            ];

            string jsonString = JsonSerializer.Serialize(blogPosts, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(FilePath, jsonString);
        }

        public interface ISeeding
        {
            string FilePath { get; }
            void Seed();
        }
    }
}