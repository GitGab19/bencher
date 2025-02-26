import { For, type Accessor, Switch, Match } from "solid-js";
import { PerfTab } from "../../../../config/types";
import type {
	JsonBenchmark,
	JsonBranch,
	JsonReport,
	JsonTestbed,
} from "../../../../types/bencher";
import { fmtDateTime, toCapitalized } from "../../../../config/util";
import { DEFAULT_PAGE } from "../PerfPanel";
import Pagination, { PaginationSize } from "../../../site/Pagination";

export type TabList<T> = TabElement<T>[];

export interface TabElement<T> {
	resource: T;
	checked: boolean;
}

const perf_tabs = [
	PerfTab.REPORTS,
	PerfTab.BRANCHES,
	PerfTab.TESTBEDS,
	PerfTab.BENCHMARKS,
];

export interface Props {
	project_slug: Accessor<undefined | string>;
	isConsole: boolean;
	metric_kind: Accessor<undefined | string>;
	tab: Accessor<PerfTab>;
	handleTab: (tab: PerfTab) => void;
	// Tabs
	reports_tab: TabList<JsonReport>;
	branches_tab: TabList<JsonBranch>;
	testbeds_tab: TabList<JsonTestbed>;
	benchmarks_tab: TabList<JsonBenchmark>;
	// Per page
	reports_per_page: Accessor<number>;
	branches_per_page: Accessor<number>;
	testbeds_per_page: Accessor<number>;
	benchmarks_per_page: Accessor<number>;
	// Page
	reports_page: Accessor<number>;
	branches_page: Accessor<number>;
	testbeds_page: Accessor<number>;
	benchmarks_page: Accessor<number>;
	// Handle checked
	handleReportChecked: (
		index: number,
		metric_kind_slug: undefined | string,
	) => void;
	handleBranchChecked: (index: number) => void;
	handleTestbedChecked: (index: number) => void;
	handleBenchmarkChecked: (index: number) => void;
	// Handle page
	handleReportsPage: (reports_page: number) => void;
	handleBranchesPage: (branches_page: number) => void;
	handleTestbedsPage: (testbeds_page: number) => void;
	handleBenchmarksPage: (benchmarks_page: number) => void;
}

const PlotTab = (props: Props) => {
	const getTab = () => {
		switch (props.tab()) {
			case PerfTab.REPORTS:
				return props.reports_tab;
			case PerfTab.BRANCHES:
				return props.branches_tab;
			case PerfTab.TESTBEDS:
				return props.testbeds_tab;
			case PerfTab.BENCHMARKS:
				return props.benchmarks_tab;
			default:
				return [];
		}
	};

	const getPerPage = () => {
		switch (props.tab()) {
			case PerfTab.REPORTS:
				return props.reports_per_page;
			case PerfTab.BRANCHES:
				return props.branches_per_page;
			case PerfTab.TESTBEDS:
				return props.testbeds_per_page;
			case PerfTab.BENCHMARKS:
				return props.benchmarks_per_page;
			default:
				return () => 8;
		}
	};

	const getPage = () => {
		switch (props.tab()) {
			case PerfTab.REPORTS:
				return props.reports_page;
			case PerfTab.BRANCHES:
				return props.branches_page;
			case PerfTab.TESTBEDS:
				return props.testbeds_page;
			case PerfTab.BENCHMARKS:
				return props.benchmarks_page;
			default:
				return () => 1;
		}
	};

	const getHandlePage = () => {
		switch (props.tab()) {
			case PerfTab.REPORTS:
				return props.handleReportsPage;
			case PerfTab.BRANCHES:
				return props.handleBranchesPage;
			case PerfTab.TESTBEDS:
				return props.handleTestbedsPage;
			case PerfTab.BENCHMARKS:
				return props.handleBenchmarksPage;
			default:
				return (page: number) =>
					console.error("No handle for tab", props.tab(), page);
		}
	};

	const handleChecked = (index: number, slug?: string) => {
		switch (props.tab()) {
			case PerfTab.REPORTS:
				return props.handleReportChecked(index, slug);
			case PerfTab.BRANCHES:
				return props.handleBranchChecked(index);
			case PerfTab.TESTBEDS:
				return props.handleTestbedChecked(index);
			case PerfTab.BENCHMARKS:
				return props.handleBenchmarkChecked(index);
			default:
				return [];
		}
	};

	return (
		<>
			<div class="panel-tabs">
				<For each={perf_tabs}>
					{(tab, index) => (
						<>
							<a
								class={props.tab() === tab ? "is-active" : ""}
								title={`View ${toCapitalized(tab)}`}
								onClick={() => props.handleTab(tab)}
							>
								{toCapitalized(tab)}
							</a>
							{index() === 0 && (
								<a style="pointer-events:none;color:#fdb07e;">||</a>
							)}
						</>
					)}
				</For>
			</div>
			<Tab
				project_slug={props.project_slug}
				isConsole={props.isConsole}
				metric_kind={props.metric_kind}
				tab={props.tab}
				getTab={getTab}
				getPage={getPage}
				getHandlePage={getHandlePage}
				handleChecked={handleChecked}
			/>
			<div class="panel-block">
				<div class="container">
					<div class="columns is-centered">
						<div class="column is-11">
							<br />
							<Pagination
								size={PaginationSize.SMALL}
								data_len={getTab()?.length}
								per_page={getPerPage()?.()}
								page={getPage()?.()}
								handlePage={getHandlePage()}
							/>
							<br />
						</div>
					</div>
				</div>
			</div>
		</>
	);
};

const Tab = (props: {
	project_slug: Accessor<undefined | string>;
	isConsole: boolean;
	metric_kind: Accessor<undefined | string>;
	tab: Accessor<PerfTab>;
	getTab: () => TabList<JsonReport | JsonBranch | JsonTestbed | JsonBenchmark>;
	getPage: () => Accessor<number>;
	getHandlePage: () => (page: number) => void;
	handleChecked: (index: number, slug?: string) => void;
}) => {
	return (
		<Switch
			fallback={
				<div class="box">
					<p>No {props.tab()} found</p>
				</div>
			}
		>
			<Match
				when={
					props.isConsole &&
					props.getTab().length === 0 &&
					props.getPage()?.() === DEFAULT_PAGE
				}
			>
				<div class="box">
					<div class="columns is-centered">
						<div class="column is-5">
							<AddButton project_slug={props.project_slug} tab={props.tab} />
						</div>
					</div>
				</div>
			</Match>
			<Match
				when={
					props.getTab().length === 0 && props.getPage()?.() !== DEFAULT_PAGE
				}
			>
				<div class="box">
					<div class="columns is-centered">
						<div class="column is-5">
							<BackButton
								tab={props.tab}
								page={props.getPage()}
								handlePage={props.getHandlePage()}
							/>
						</div>
					</div>
				</div>
			</Match>
			<Match
				when={props.tab() === PerfTab.REPORTS && props.getTab().length > 0}
			>
				<For each={props.getTab()}>
					{(report, index) => (
						<For each={(report.resource as JsonReport)?.results?.[0]}>
							{(result, _index) => (
								<a
									class="panel-block"
									title={`View Report from ${fmtDateTime(
										(report.resource as JsonReport)?.start_time,
									)}`}
									onClick={(_e) =>
										// Send the Metric Kind slug instead of the Report UUID
										props.handleChecked(index(), result.metric_kind?.slug)
									}
								>
									<div class="columns is-vcentered is-mobile">
										<div class="column is-narrow">
											<input
												type="radio"
												checked={
													report.checked &&
													result.metric_kind?.slug === props.metric_kind()
												}
											/>
										</div>
										<div class="column">
											<small style="overflow-wrap:anywhere;">
												{fmtDateTime(
													(report.resource as JsonReport)?.start_time,
												)}
											</small>
											<ReportDimension
												icon="fas fa-shapes"
												name={result.metric_kind?.name}
											/>
											<ReportDimension
												icon="fas fa-code-branch"
												name={(report.resource as JsonReport)?.branch?.name}
											/>
											<ReportDimension
												icon="fas fa-server"
												name={(report.resource as JsonReport)?.testbed?.name}
											/>
										</div>
									</div>
								</a>
							)}
						</For>
					)}
				</For>
			</Match>
			<Match
				when={props.tab() !== PerfTab.REPORTS && props.getTab().length > 0}
			>
				<For each={props.getTab()}>
					{(dimension, index) => (
						<a
							class="panel-block"
							title={`${dimension.checked ? "Remove" : "Add"} ${
								(dimension.resource as JsonBranch | JsonTestbed | JsonBenchmark)
									?.name
							}`}
							onClick={(_e) => props.handleChecked(index())}
						>
							<div class="columns is-vcentered is-mobile">
								<div class="column is-narrow">
									<input type="checkbox" checked={dimension.checked} />
								</div>
								<div class="column">
									<small style="overflow-wrap:anywhere;">
										{
											(
												dimension.resource as
													| JsonBranch
													| JsonTestbed
													| JsonBenchmark
											)?.name
										}
									</small>
								</div>
							</div>
						</a>
					)}
				</For>
			</Match>
		</Switch>
	);
};

const AddButton = (props: {
	project_slug: Accessor<undefined | string>;
	tab: Accessor<PerfTab>;
}) => {
	const getHref = () => {
		switch (props.tab()) {
			case PerfTab.BRANCHES:
			case PerfTab.TESTBEDS:
				return `/console/projects/${props.project_slug()}/${props.tab()}/add`;
			case PerfTab.REPORTS:
			case PerfTab.BENCHMARKS:
				return "/docs/how-to/track-benchmarks";
			default:
				return "#";
		}
	};

	const getText = () => {
		switch (props.tab()) {
			case PerfTab.BRANCHES:
				return "Add a Branch";
			case PerfTab.TESTBEDS:
				return "Add a Testbed";
			case PerfTab.REPORTS:
			case PerfTab.BENCHMARKS:
				return "Track Your Benchmarks";
			default:
				return "Unknown Tab";
		}
	};

	return (
		<a
			class="button is-primary is-fullwidth"
			title={getText()}
			href={getHref()}
		>
			{getText()}
		</a>
	);
};

const BackButton = (props: {
	tab: Accessor<PerfTab>;
	page: Accessor<number>;
	handlePage: (page: number) => void;
}) => {
	return (
		<button
			class="button is-primary is-fullwidth"
			title="Go back to the previous page"
			onClick={(e) => {
				e.preventDefault();
				props.handlePage(props.page() - 1);
			}}
		>
			That's all the {props.tab()}. Go back.
		</button>
	);
};

const ReportDimension = (props: { icon: string; name: string }) => {
	return (
		<div>
			<span class="icon">
				<i class={props.icon} aria-hidden="true" />
			</span>
			<small style="overflow-wrap:anywhere;">{props.name}</small>
		</div>
	);
};

export default PlotTab;
