import {
	Accessor,
	createEffect,
	createMemo,
	createResource,
	For,
} from "solid-js";
import Pagination, { PaginationSize } from "../site/Pagination";
import { useSearchParams } from "../../util/url";
import { validU32 } from "../../util/valid";
import { authUser } from "../../util/auth";
import type { JsonProject } from "../../types/bencher";
import { httpGet } from "../../util/http";

// const SORT_PARAM = "sort";
// const DIRECTION_PARAM = "direction";
const PER_PAGE_PARAM = "per_page";
const PAGE_PARAM = "page";

const DEFAULT_PER_PAGE = 8;
const DEFAULT_PAGE = 1;

export interface Props {
	apiUrl: string;
}

const PublicProjects = (props: Props) => {
	const [searchParams, setSearchParams] = useSearchParams();

	const initParams: Record<string, number> = {};
	if (!validU32(searchParams[PER_PAGE_PARAM])) {
		initParams[PER_PAGE_PARAM] = DEFAULT_PER_PAGE;
	}
	if (!validU32(searchParams[PAGE_PARAM])) {
		initParams[PAGE_PARAM] = DEFAULT_PAGE;
	}
	if (Object.keys(initParams).length !== 0) {
		setSearchParams(initParams);
	}

	const per_page = createMemo(() => Number(searchParams[PER_PAGE_PARAM]));
	const page = createMemo(() => Number(searchParams[PAGE_PARAM]));

	const pagination = createMemo(() => {
		return {
			per_page: per_page(),
			page: page(),
			public: true,
		};
	});
	const fetcher = createMemo(() => {
		return {
			pagination: pagination(),
			token: authUser()?.token,
		};
	});
	const fetchProjects = async (fetcher: {
		pagination: {
			per_page: number;
			page: number;
			public: boolean;
		};
		token: string | undefined;
	}) => {
		const EMPTY_ARRAY: JsonProject[] = [];
		const searchParams = new URLSearchParams();
		for (const [key, value] of Object.entries(fetcher.pagination)) {
			if (value) {
				searchParams.set(key, value.toString());
			}
		}
		const path = `/v0/projects?${searchParams.toString()}`;
		return await httpGet(props.apiUrl, path, fetcher.token)
			.then((resp) => resp?.data)
			.catch((error) => {
				console.error(error);
				return EMPTY_ARRAY;
			});
	};
	const [projects] = createResource<JsonProject[]>(fetcher, fetchProjects);

	createEffect(() => {
		const newParams: Record<string, number> = {};
		if (!validU32(searchParams[PER_PAGE_PARAM])) {
			newParams[PER_PAGE_PARAM] = DEFAULT_PER_PAGE;
		}
		if (!validU32(searchParams[PAGE_PARAM])) {
			newParams[PAGE_PARAM] = DEFAULT_PAGE;
		}
		if (Object.keys(newParams).length !== 0) {
			setSearchParams(newParams);
		}
	});

	const handlePage = (page: number) => {
		if (validU32(page)) {
			setSearchParams({ [PAGE_PARAM]: page });
		}
	};

	return (
		<section class="section">
			<div class="container">
				<div class="columns is-mobile">
					<div class="column">
						<div class="content">
							<h2 class="title">Projects</h2>
							<hr />
							<br />
							<For each={projects()}>
								{(project) => (
									<a
										class="box"
										title={`View ${project.name}`}
										href={`/perf/${project.slug}`}
									>
										{project.name}
									</a>
								)}
							</For>
							{projects()?.length === 0 && page() !== 1 && (
								<div class="box">
									<BackButton page={page} handlePage={handlePage} />
								</div>
							)}
							<br />
						</div>
					</div>
				</div>
				<Pagination
					size={PaginationSize.REGULAR}
					data_len={projects()?.length}
					per_page={per_page()}
					page={page()}
					handlePage={handlePage}
				/>
			</div>
		</section>
	);
};

const BackButton = (props: {
	page: Accessor<number>;
	handlePage: (page: number) => void;
}) => {
	return (
		<button
			class="button is-primary is-fullwidth"
			onClick={(e) => {
				e.preventDefault();
				props.handlePage(props.page() - 1);
			}}
		>
			That's all the projects. Go back.
		</button>
	);
};

export default PublicProjects;
