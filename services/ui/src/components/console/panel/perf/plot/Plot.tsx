import { createElementSize } from "@solid-primitives/resize-observer";
import { createMemo, createResource, createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import LinePlot from "./LinePlot";
import PlotKey from "./PlotKey";
import { BENCHER_LOGO_URL } from "../../../../site/util";

const Plot = (props) => {
	const [perf_active, setPerfActive] = createStore([]);

	const [_perf_active] = createResource(props.perf_data, (json_perf) => {
		const active = [];
		json_perf?.results?.forEach(() => {
			active.push(true);
		});
		setPerfActive(active);
		return active;
	});

	const handlePerfActive = (index: number) => {
		const active = [...perf_active];
		active[index] = !active[index];
		setPerfActive(active);
	};

	let plot_ref: HTMLDivElement | undefined;
	const plot_size = createElementSize(() => plot_ref);
	const width = createMemo(() => plot_size.width);

	const [wordmarkId, setWordmarkId] = createSignal("loading_wordmark");

	return (
		<div class="container">
			{props.img() && (
				<>
					<img
						id={wordmarkId()}
						onLoad={(_e) => setWordmarkId("perf_wordmark")}
						src={BENCHER_LOGO_URL}
						width="152"
						height="28"
						alt="🐰 Bencher"
					/>
					<br />
					<br />
				</>
			)}
			<div ref={(e) => (plot_ref = e)}>
				<LinePlot
					user={props.user}
					config={props.config}
					path_params={props.path_params}
					perf_data={props.perf_data}
					perf_active={perf_active}
					width={width}
				/>
			</div>
			<PlotKey
				user={props.user}
				config={props.config}
				path_params={props.path_params}
				branches={props.branches}
				testbeds={props.testbeds}
				benchmarks={props.benchmarks}
				perf_data={props.perf_data}
				key={props.key}
				perf_active={perf_active}
				handleKey={props.handleKey}
				handlePerfActive={handlePerfActive}
			/>
		</div>
	);
};

export default Plot;
