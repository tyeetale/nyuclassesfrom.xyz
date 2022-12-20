import { instantMeiliSearch } from "@meilisearch/instant-meilisearch";
import "instantsearch.css/themes/satellite-min.css";
import { useRouter } from "next/router";
import { useState } from "react";
import {
  Highlight,
  Hits,
  InstantSearch,
  Pagination,
  SearchBox,
  Stats,
} from "react-instantsearch-dom";

const searchClient = instantMeiliSearch(
  "https://ms-53f05ab45a7a-999.nyc.meilisearch.io",
  process.env.NEXT_PUBLIC_MEILI_KEY
);

type HitProps = {
  at: string;
  class_name: string;
  class_number: number;
  class_status: string;
  component: string;
  course_location: string;
  description: string;
  end_time: string;
  fulfillment: string;
  grading: string;
  id: number;
  instruction_mode: string;
  instructors: string[];
  meet_friday: boolean;
  meet_monday: boolean;
  meet_saturday: boolean;
  meet_sunday: boolean;
  meet_thursday: boolean;
  meet_tuesday: boolean;
  meet_wednesday: boolean;
  notes: string;
  prerequisite: string;
  school_name: string;
  section: string;
  session_end: string;
  session_start: string;
  start_time: string;
  subject_code: string;
  subject_name: string;
  subject_number: string;
  term: string;
  timezone: string;
  units: number;
  year: number;
};

const Search = () => {
  const router = useRouter();
  const [searchQuery, setSearchQuery] = useState(router.query.search as string);
  const Hit = ({ hit }: any) => {
    return (
      <div className=" py-3">
        <h1 className="text-xl font-bold">
          <Highlight attribute="class_name" hit={hit} /> (
          <Highlight attribute="term" hit={hit} />{" "}
          <Highlight attribute="year" hit={hit} />)
        </h1>
        <h2 className="text-lg">
          #<Highlight attribute="class_number" hit={hit} /> |{" "}
          <Highlight attribute="school_name" hit={hit} /> | Units:{" "}
          <Highlight attribute="units" hit={hit} />
          {hit.session_start ? (
            <>
              {" "}
              | <Highlight attribute="session_start" hit={hit} />
            </>
          ) : null}
        </h2>
        <h3 className="text-base">
          <Highlight attribute="section" hit={hit} /> |{" "}
          <Highlight attribute="instructors" hit={hit} /> |{" "}
          <Highlight attribute="grading" hit={hit} /> |{" "}
          <Highlight attribute="instruction_mode" hit={hit} /> |{" "}
          <Highlight attribute="course_location" hit={hit} /> |{" "}
          <Highlight attribute="component" hit={hit} />
        </h3>
        <p className="text-sm">
          <Highlight attribute="description" hit={hit} />
        </p>
      </div>
    );
  };

  return (
    <div className="container mx-auto p-8 px-10">
      <h1 className="font-extrabold text-transparent text-2xl bg-clip-text bg-gradient-to-r from-purple-600 to-pink-600">
        nyuclassesfrom.xyz
      </h1>

      <InstantSearch indexName="course-fa2022" searchClient={searchClient}>
        <SearchBox />
        <Stats />
        <Hits hitComponent={Hit} />
        <Pagination showLast={true} />
      </InstantSearch>
    </div>
  );
};

export default Search;
