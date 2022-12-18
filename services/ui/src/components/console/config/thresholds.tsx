import THRESHOLD_FIELDS from "./fields/threshold";
import { BENCHER_API_URL } from "../../site/util";
import { Button, Card, Display, Operation, Row } from "./types";
import { parentPath, addPath, viewUuidPath } from "./util";
import FieldKind from "../../field/kind";

const TEST_VALUE = {
  selected: "z",
  options: [
    {
      value: "z",
      option: "Z-Score",
    },
    {
      value: "t",
      option: "Student's t-test",
    },
  ],
};

const thresholdsConfig = {
  [Operation.LIST]: {
    operation: Operation.LIST,
    header: {
      title: "Thresholds",
      buttons: [
        {
          kind: Button.ADD,
          path: addPath,
        },
        { kind: Button.REFRESH },
      ],
    },
    table: {
      url: (path_params) =>
        `${BENCHER_API_URL()}/v0/projects/${
          path_params?.project_slug
        }/thresholds`,
      add: {
        path: addPath,
        text: "Add a Threshold",
      },
      row: {
        key: "uuid",
        items: [
          {
            kind: Row.FOREIGN,
            key: "branch",
          },
          {
            kind: Row.FOREIGN,
            key: "testbed",
          },
          {
            kind: Row.FOREIGN,
            key: "metric_kind",
          },
          {},
        ],
        button: {
          text: "View",
          path: (pathname, datum) => viewUuidPath(pathname, datum),
        },
      },
    },
  },
  [Operation.ADD]: {
    operation: Operation.ADD,
    header: {
      title: "Add Threshold",
      path: parentPath,
    },
    form: {
      url: (path_params) =>
        `${BENCHER_API_URL()}/v0/projects/${
          path_params?.project_slug
        }/thresholds`,
      fields: [
        {
          kind: FieldKind.HIDDEN,
          key: "project",
          path_param: "project_slug",
        },
        {
          kind: FieldKind.RADIO,
          label: "Branch",
          key: "branch",
          value: "",
          valid: null,
          validate: true,
          nullify: false,
          clear: false,
          config: THRESHOLD_FIELDS.branch,
        },
        {
          kind: FieldKind.RADIO,
          label: "Testbed",
          key: "testbed",
          value: "",
          valid: null,
          validate: true,
          nullify: false,
          clear: false,
          config: THRESHOLD_FIELDS.testbed,
        },
        {
          kind: FieldKind.RADIO,
          label: "Metric Kind",
          key: "metric_kind",
          value: "",
          valid: null,
          validate: true,
          nullify: false,
          clear: false,
          config: THRESHOLD_FIELDS.metric_kind,
        },
        {
          kind: FieldKind.SELECT,
          label: "Test Type",
          key: "test",
          value: TEST_VALUE,
          validate: false,
          nullify: false,
          clear: false,
          config: THRESHOLD_FIELDS.test,
        },
      ],
      path: parentPath,
    },
  },
  [Operation.VIEW]: {
    operation: Operation.VIEW,
    header: {
      key: "uuid",
      path: parentPath,
    },
    deck: {
      url: (path_params) =>
        `${BENCHER_API_URL()}/v0/projects/${
          path_params?.project_slug
        }/thresholds/${path_params?.threshold_uuid}`,
      cards: [
        {
          kind: Card.FIELD,
          label: "Branch UUID",
          key: "branch",
          display: Display.RAW,
        },
        {
          kind: Card.FIELD,
          label: "Testbed UUID",
          key: "testbed",
          display: Display.RAW,
        },
        {
          kind: Card.FIELD,
          label: "Metric Kind UUID",
          key: "metric_kind",
          display: Display.RAW,
        },
      ],
      buttons: false,
    },
  },
};

export default thresholdsConfig;
