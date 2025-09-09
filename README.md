# Developer Blog

ASP Dotnet backend for a blogging site.

## Running Developer Blog

> cd ~/Developer-Blog/Server
>
> dotnet restore
>
> dotnet build
>
> dotnet run

Or set Server.csproj as the launch project in your IDE.

## Testing Developer Blog

> cd ~/Developer-Blog/ServerTests
>
> dotnet restore
>
> dotnet test

Or set the ServerTests.csproj up in your testing enviroment in your IDE of choice.

## Dependencies

Follow the steps for installing dotnet 9.0 runtime for your given operating system.

> <https://dotnet.microsoft.com/download/dotnet/9.0>

## RESTFUL

### GET

> GET <http://localhost:5000/blogs/all>
>
> GET <http://localhost:5000/blogs/?id=2>

### POST

> POST <http://localhost:5000/blogs>

with BODY

```json
{
    "Id": 8,
    "Title": "No I REALLY like blogs",
    "Content": "This blog writter is serious!",
    "AuthorId": "Jeff",
    "AuthorEmail": "Jeff@hello.com",
    "IsPublished": true
}
```

### DELETE

> DELETE <http://localhost:5000/blogs/?id=8>

### PUT

> PUT <http://localhost:5000/blogs/>

```json
{
    "Id": 8,
    "Title": "Updated",
    "Content": "Updated",
    "AuthorId": "Jeff",
    "AuthorEmail": "Jeff@hello.com",
    "IsPublished": true
}
```
