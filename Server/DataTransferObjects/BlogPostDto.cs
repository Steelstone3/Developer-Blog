namespace Server.DataTransferObjects
{
    public class BlogPostDto
    {
        public BlogPostDto(int id)
        {
            Id = id;
        }
        
        public int Id { get; set; }
    }
}