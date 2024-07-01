using Aspire.Hosting;

var builder = DistributedApplication.CreateBuilder(args);

builder.AddProject<Projects.Playground_WebApi>("api");
builder.AddNpmApp("frontend", "../Playground.Frontend", "start").PublishAsDockerFile();

builder.Build().Run();
