const core = require("@actions/core");
const github = require("@actions/github");

try {
  //   throw new Error("an error");

  //   core.debug("Debug message");
  //   core.warning("Warning message");
  //   core.error("Error message");

  const token = core.getInput("token");
  const title = core.getInput("title");
  const body = core.getInput("body");
  const assignees = core.getInput("assignees");

  //core.setSecret(token);

  async function run() {
    const octokit = github.getOctokit(token);

    const { data: response } = await octokit.rest.issues.create({
      ...github.context.repo,
      title,
      body,
      assignees: assignees ? assignees.split("\n") : undefined,
    });

    core.startGroup("Response Data");
    console.log(JSON.stringify(response));
    core.endGroup();

    core.setOutput("issue", JSON.stringify(response.data));
  }
  run();
} catch (error) {
  core.setFailed(error.message);
}
