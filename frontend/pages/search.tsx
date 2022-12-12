import { instantMeiliSearch } from "@meilisearch/instant-meilisearch";

import { Hits, InstantSearch, SearchBox, Stats } from "react-instantsearch-dom";

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
  const Hit = ({ hit }: any) => {
    return (
      <div className="border">
        <h1 className="text-xl font-bold">
          {hit.class_name} ({hit.term} {hit.year})
        </h1>
        <h2 className="text-lg">
          Class#: {hit.class_number} | School: {hit.school_name} | Units:{" "}
          {hit.units} | {hit.session_start}
        </h2>
        <h3 className="text-base">
          {hit.section} | {hit.instructors[0]} | {hit.grading} |{" "}
          {hit.instruction_mode} | {hit.course_location} | {hit.component}
        </h3>
        <p className="text-sm">{hit.description}</p>
      </div>
    );
  };

  return (
    <div>
      <h1>NYU Classes From NYU</h1>
      <InstantSearch indexName="course-fa2022" searchClient={searchClient}>
        <SearchBox />
        <Stats />
        <Hits hitComponent={Hit} />
      </InstantSearch>
    </div>
  );
};

export default Search;
