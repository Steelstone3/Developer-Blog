using System.Text.Json;
using Server.Models;

public class Seeding
{
    public void Seed(string filePath)
    {
        List<BlogPost> posts =
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

        string jsonString = JsonSerializer.Serialize(posts, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(filePath, jsonString);
    }
}