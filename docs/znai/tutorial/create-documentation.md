This tutorial walks you through creating and deploying [Znai](https://testingisdocumenting.org/znai) documentation as GitHub pages.

# Prerequisites

You have a GitHub project named `my-project`. This project is not java-based. 
Despite not dealing with java you have JDK and Maven installed.

# Set up directory and build file

Create a `docs` directory. It will be a place where your documentation resides. Znai will create a `znai` directory inside the `docs` directory.

In the `docs` directory create a `pom.xml` file with the znai plugin

:include-xml: pom.xml {paths: ["project.build[0].plugins.plugin.configuration[0].docId"]}


Note: Make sure that the `docId` is the same as the name of your project.
Znai assumes that the site is deployed with some common prefix. It will build HTML in a way it tries to use the `https://<username>.github.io` as a root. You need document root to correctly serve files from `https://<username>.github.io/<project-name>`.

# Scaffold the structure for znai project

In the `docs` directory run maven command to scaffold the documentation.

```shell
mvn znai:new
```

The command creates znai project structure in `docs/znai` directory.

# Edit your documentation

Open live preview of your znai site. In the `docs` directory run the command.

```shell
mvn znai:preview
```

Follow the guidelines on the [znai project website](https://testingisdocumenting.org/znai).

