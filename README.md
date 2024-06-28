# Cloudracer

This tool helps you generate build and deployment script for .NET Aspire based projects so you can deploy them as
container apps on Azure. We use this tool for our digital GPT assistant applications to speed up the workflow.

## Getting started

```
pip install cloudracer
```

### Generating build scripts

Generating build scripts can be done by running `cloudracer generate build --project <project-path>`. This produces
github action files for the project based on the host project. This is a one time generation step, after
the scripts are generated you can modify them as you see fit.

### Generating deployment scripts

You can generate deployment scripts by running `cloudracer generate deployment --project <project-path>`. This
produces a set of bicep scripts for the application. You can use these bicep scripts to deploy revisions of your
application to an Azure Container App environment of choice.

## Developing

### System requirements

- [Rye](https://rye-up.com)
- [Azure CLI](https://learn.microsoft.com/en-us/cli/azure/)
- [.NET 8 SDK](https://dot.net)

### Running tests

We use pytest to run unit-tests and integration tests. You can run the tests using the following commands:

```bash
python -m pytest .
```
