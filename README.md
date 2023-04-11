# checkmigrations
A lightweight CLI tool used to check if there are any conflicts with a project's migrations for a database. 

## How to use
Once installed you can run the following command to check if there are any conflicts with the migrations files based on the framework given:
```
checkmigrations <framework> <path to project>
```

example with a relative path to a Django project:
```
checkmigrations django ./path/to/project
```
example with an absolute path to a Django project:
```
checkmigrations django /path/to/project
```

## Supported Frameworks
- Django

If you wish to know more about the project, please check out the [docs](docs/development.md).