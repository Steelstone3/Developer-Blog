namespace Server.Models
{
    public class BlogPost
    {
        public BlogPost(int id, string title, string content, string authorId, string authorEmail, bool isPublished)
        {
            Id = id;
            Title = title;
            Content = content;
            AuthorId = authorId;
            AuthorEmail = authorEmail;
            IsPublished = isPublished;
        }

        public int Id { get; set; }
        public string Title { get; set; }
        public string Content { get; set; }
        public string AuthorId { get; set; }
        public string AuthorEmail { get; set; }
        public bool IsPublished { get; set; }
    }
}