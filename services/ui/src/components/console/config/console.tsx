import { Resource } from "./types";
import organizationsConfig from "./organizations";
import projectsConfig from "./projects";
import testbedsConfig from "./testbeds";
import branchesConfig from "./branches";
import reportsConfig from "./reports";
import thresholdsConfig from "./thresholds";
import alertsConfig from "./alerts";
import membersConfig from "./members";

const consoleConfig = {
  [Resource.ORGANIZATIONS]: organizationsConfig,
  [Resource.MEMBERS]: membersConfig,
  [Resource.PROJECTS]: projectsConfig,
  [Resource.REPORTS]: reportsConfig,
  [Resource.BRANCHES]: branchesConfig,
  [Resource.TESTBEDS]: testbedsConfig,
  [Resource.THRESHOLDS]: thresholdsConfig,
  [Resource.ALERTS]: alertsConfig,
};

export default consoleConfig;
