# Enable GitHub pages

Before publishing documentation you must enable pages support for your repository.
We'll use actions to build and publish static documentation.

* Go to your project settings
* Navigate to the *Pages* section
* In *Source* section change source to *GitHub actions*


# Set up the pipeline

Create an action file in `.github/workflows/znai-pages-deploy.yml`.

Use the following example to build and deploy your pages.

:include-file: znai-pages-deploy.yaml

Warning: Make sure you changed the path in `Upload artifact` step. It must be set to your repository name.

:include-file: znai-pages-deploy.yaml {startLine: "Upload artifact", endLine: "path:", highlight: "path:"}

# Checking documentation

Make changes in documentation. Commit and push your changes.

Go to the *Actions* tab of your repository. Wait for "Publish Documentation" job to finish.

After that navigate to the GitHub pages of your project.