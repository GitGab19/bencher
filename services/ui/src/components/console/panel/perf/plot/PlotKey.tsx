import axios from "axios";
import { createMemo, createResource, createSignal, For } from "solid-js";
import { PerfTab } from "../../../config/types";
import * as d3 from "d3";
import { get_options } from "../../../../site/util";
import { createStore } from "solid-js/store";

const PlotKey = (props) => {
	const branches_fetcher = createMemo(() => {
		return {
			branches: props.branches(),
			token: props.user?.token,
		};
	});
	const testbeds_fetcher = createMemo(() => {
		return {
			testbeds: props.testbeds(),
			token: props.user?.token,
		};
	});
	const benchmarks_fetcher = createMemo(() => {
		return {
			benchmarks: props.benchmarks(),
			token: props.user?.token,
		};
	});

	const getLs = async (perf_tab: PerfTab, fetcher) => {
		const key_data = {};

		await Promise.all(
			fetcher[perf_tab]?.map(async (uuid: string) => {
				const url = props.config?.key_url(props.path_params(), perf_tab, uuid);
				await axios(get_options(url, fetcher.token))
					.then((resp) => {
						key_data[uuid] = resp.data;
					})
					.catch(console.error);
			}),
		);

		return key_data;
	};

	const [dimensions, setDimensions] = createStore({
		branches: false,
		testbeds: false,
		benchmarks: false,
	});
	const perf_key = createMemo(() => {
		if (dimensions.branches && dimensions.testbeds && dimensions.benchmarks) {
			return "perf_key";
		} else {
			return "loading_key";
		}
	});
	const dimension_fetcher = async (tab: PerfTab, fetcher) => {
		const dimension = await getLs(tab, fetcher);
		setDimensions({ ...dimensions, [tab]: true });
		return dimension;
	};
	const [branches] = createResource(branches_fetcher, async (fetcher) =>
		dimension_fetcher(PerfTab.BRANCHES, fetcher),
	);
	const [testbeds] = createResource(testbeds_fetcher, async (fetcher) =>
		dimension_fetcher(PerfTab.TESTBEDS, fetcher),
	);
	const [benchmarks] = createResource(benchmarks_fetcher, async (fetcher) =>
		dimension_fetcher(PerfTab.BENCHMARKS, fetcher),
	);

	return (
		<>
			{props.key() ? (
				<ExpandedKey
					perf_key={perf_key}
					branches={branches}
					testbeds={testbeds}
					benchmarks={benchmarks}
					perf_data={props.perf_data}
					perf_active={props.perf_active}
					handleKey={props.handleKey}
					handlePerfActive={props.handlePerfActive}
				/>
			) : (
				<MinimizedKey
					perf_data={props.perf_data}
					perf_active={props.perf_active}
					handleKey={props.handleKey}
					handlePerfActive={props.handlePerfActive}
				/>
			)}
		</>
	);
};

const ExpandedKey = (props) => {
	return (
		<div
			class="columns is-centered is-vcentered is-gapless is-multiline"
			id={props.perf_key()}
		>
			<div class="column is-narrow">
				<MinimizeKeyButton handleKey={props.handleKey} />
			</div>
			<For each={props.perf_data()?.results}>
				{(
					result: {
						branch: string;
						testbed: string;
						benchmark: string;
					},
					index,
				) => (
					<div class="column is-2">
						<KeyResource
							icon="fas fa-code-branch"
							name={props.branches()?.[result.branch]?.name}
						/>
						<KeyResource
							icon="fas fa-server"
							name={props.testbeds()?.[result.testbed]?.name}
						/>
						<KeyResource
							icon="fas fa-tachometer-alt"
							name={props.benchmarks()?.[result.benchmark]?.name}
						/>
						<KeyButton
							index={index}
							perf_active={props.perf_active}
							handlePerfActive={props.handlePerfActive}
						/>
					</div>
				)}
			</For>
			<div class="column is-narrow">
				<MinimizeKeyButton handleKey={props.handleKey} />
			</div>
		</div>
	);
};

const MinimizedKey = (props) => {
	return (
		<div class="columns is-centered is-vcentered is-gapless is-multiline is-mobile">
			<div class="column is-narrow">
				<MaximizeKeyButton handleKey={props.handleKey} />
			</div>
			<For each={props.perf_data()?.results}>
				{(_result, index) => (
					<div class="column is-narrow">
						<KeyButton
							index={index}
							perf_active={props.perf_active}
							handlePerfActive={props.handlePerfActive}
						/>
					</div>
				)}
			</For>
			<div class="column is-narrow">
				<MaximizeKeyButton handleKey={props.handleKey} />
			</div>
		</div>
	);
};

const MinimizeKeyButton = (props) => {
	return (
		<button
			class="button is-small is-fullwidth is-primary is-inverted"
			onClick={() => props.handleKey(false)}
		>
			<span class="icon">
				<i class="fas fa-minus" aria-hidden="true" />
			</span>
		</button>
	);
};

const MaximizeKeyButton = (props) => {
	return (
		<button
			class="button is-small is-fullwidth is-primary is-inverted"
			onClick={() => props.handleKey(true)}
		>
			<span class="icon">
				<i class="fas fa-plus" aria-hidden="true" />
			</span>
		</button>
	);
};

const KeyResource = (props) => {
	return (
		<div>
			<span class="icon">
				<i class={props.icon} aria-hidden="true" />
			</span>
			<small style="overflow-wrap:anywhere;">{` ${
				props.name ? props.name : "Loading..."
			}`}</small>
		</div>
	);
};

const KeyButton = (props) => {
	const color = d3.schemeTableau10[props.index() % 10];

	return (
		<button
			// On click toggle visibility of key
			// move button over to being is-outlined
			class="button is-small is-fullwidth"
			style={
				props.perf_active[props.index()]
					? `background-color:${color};`
					: `border-color:${color};color:${color};`
			}
			onClick={() => props.handlePerfActive(props.index())}
		>
			{props.index() + 1}
		</button>
	);
};

export default PlotKey;
