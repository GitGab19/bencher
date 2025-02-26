import { createMemo, Match, Switch } from "solid-js";
import TablePanel from "./table/TablePanel";
import DeckPanel from "./deck/DeckPanel";
import { Operation } from "../config/types";
import PerfPanel from "./perf/PerfPanel";
import PosterPanel from "./poster/PosterPanel";
import { useLocation } from "solid-app-router";
import BillingPanel from "./billing/BillingPanel";
import HelpPanel from "./help/HelpPanel";

const ConsolePanel = (props) => {
	const location = useLocation();
	const pathname = createMemo(() => location.pathname);

	return (
		<Switch fallback={<p>Unknown console path: {pathname()} </p>}>
			<Match when={props.config?.operation === Operation.LIST}>
				<TablePanel
					user={props.user}
					config={props.config}
					path_params={props.path_params}
				/>
			</Match>
			<Match when={props.config?.operation === Operation.ADD}>
				<PosterPanel
					user={props.user}
					config={props.config}
					path_params={props.path_params}
				/>
			</Match>
			<Match when={props.config?.operation === Operation.VIEW}>
				<DeckPanel
					user={props.user}
					config={props.config}
					path_params={props.path_params}
				/>
			</Match>
			<Match when={props.config?.operation === Operation.EDIT}>
				<PosterPanel
					user={props.user}
					config={props.config}
					path_params={props.path_params}
				/>
			</Match>
			<Match when={props.config?.operation === Operation.PERF}>
				<PerfPanel
					user={props.user}
					config={props.config}
					path_params={props.path_params}
					is_console={true}
				/>
			</Match>
			<Match when={props.config?.operation === Operation.BILLING}>
				<BillingPanel
					user={props.user}
					organization_slug={props.organization_slug}
					config={props.config}
					path_params={props.path_params}
				/>
			</Match>
			<Match when={props.config?.operation === Operation.HELP}>
				<HelpPanel
					user={props.user}
					organization_slug={props.organization_slug}
					config={props.config}
					path_params={props.path_params}
				/>
			</Match>
		</Switch>
	);
};

export default ConsolePanel;
