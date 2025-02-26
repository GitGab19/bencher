import { Show } from "solid-js";
import Plot from "./Plot";
import PlotHeader from "./PlotHeader";
import PlotInit from "./PlotInit";
import PlotTab from "./PlotTab";

const PerfPlot = (props) => {
	return (
		<div class="columns">
			<div class="column">
				<nav class="panel">
					<PlotHeader
						user={props.user}
						config={props.config}
						path_params={props.path_params}
						is_console={props.is_console}
						is_plot_init={props.is_plot_init}
						metric_kind={props.metric_kind}
						start_date={props.start_date}
						end_date={props.end_date}
						refresh={props.refresh}
						range={props.range}
						clear={props.clear}
						lower_boundary={props.lower_boundary}
						upper_boundary={props.upper_boundary}
						handleMetricKind={props.handleMetricKind}
						handleStartTime={props.handleStartTime}
						handleEndTime={props.handleEndTime}
						handleRange={props.handleRange}
						handleClear={props.handleClear}
						handleLowerBoundary={props.handleLowerBoundary}
						handleUpperBoundary={props.handleUpperBoundary}
					/>
					<div class="panel-block">
						<Show
							when={props.is_plot_init()}
							fallback={
								<Plot
									user={props.user}
									config={props.config}
									path_params={props.path_params}
									branches={props.branches}
									testbeds={props.testbeds}
									benchmarks={props.benchmarks}
									range={props.range}
									lower_boundary={props.lower_boundary}
									upper_boundary={props.upper_boundary}
									perf_data={props.perf_data}
									key={props.key}
									handleKey={props.handleKey}
								/>
							}
						>
							<PlotInit
								metric_kind={props.metric_kind}
								branches={props.branches}
								testbeds={props.testbeds}
								benchmarks={props.benchmarks}
								handleTab={props.handleTab}
							/>
						</Show>
					</div>
					<PlotTab
						project_slug={props.project_slug}
						is_console={props.is_console}
						metric_kind={props.metric_kind}
						tab={props.tab}
						reports_tab={props.reports_tab}
						branches_tab={props.branches_tab}
						testbeds_tab={props.testbeds_tab}
						benchmarks_tab={props.benchmarks_tab}
						reports_per_page={props.reports_per_page}
						branches_per_page={props.branches_per_page}
						testbeds_per_page={props.testbeds_per_page}
						benchmarks_per_page={props.benchmarks_per_page}
						reports_page={props.reports_page}
						branches_page={props.branches_page}
						testbeds_page={props.testbeds_page}
						benchmarks_page={props.benchmarks_page}
						handleTab={props.handleTab}
						handleReportChecked={props.handleReportChecked}
						handleBranchChecked={props.handleBranchChecked}
						handleTestbedChecked={props.handleTestbedChecked}
						handleBenchmarkChecked={props.handleBenchmarkChecked}
						handleReportsPage={props.handleReportsPage}
						handleBranchesPage={props.handleBranchesPage}
						handleTestbedsPage={props.handleTestbedsPage}
						handleBenchmarksPage={props.handleBenchmarksPage}
					/>
				</nav>
			</div>
		</div>
	);
};

export default PerfPlot;
