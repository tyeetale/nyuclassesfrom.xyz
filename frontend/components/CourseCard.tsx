import { Highlight } from "react-instantsearch-dom";
import { formatSession } from "./utils/formatSession";

type CourseProps = {
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

const CourseCard = ({ hit }: any) => {
  const isCancelled = !(
    hit.meet_sunday ||
    hit.meet_monday ||
    hit.meet_tuesday ||
    hit.meet_wednesday ||
    hit.meet_thursday ||
    hit.meet_friday ||
    hit.meet_saturday
  );

  const dayNames = ["M", "Tu", "W", "Th", "F", "Sa", "Su"];

  const classDaysCheck = [
    hit.meet_monday,
    hit.meet_tuesday,
    hit.meet_wednesday,
    hit.meet_thursday,
    hit.meet_friday,
    hit.meet_saturday,
    hit.meet_sunday,
  ];

  const classDays = !isCancelled
    ? classDaysCheck
        .map((day, index) => (day ? dayNames[index] : ""))
        .filter((day) => day)
        .join(",")
    : null;

  if (!isCancelled)
    return (
      <div className="space-y-1.5">
        <h1 className="text-xl font-bold">
          <Highlight attribute="class_name" hit={hit} /> (
          <Highlight attribute="term" hit={hit} />{" "}
          <Highlight attribute="year" hit={hit} />)
        </h1>
        <h2 className="text-lg font-semibold text-neutral-700 dark:text-neutral-200">
          #
          <Highlight
            className="underline"
            attribute="class_number"
            hit={hit}
          />{" "}
          | <Highlight attribute="school_name" hit={hit} /> | Units:{" "}
          <Highlight attribute="units" hit={hit} />
          {!!hit.session_start && !!hit.session_end && (
            <>
              {" "}
              | {formatSession(hit.session_start, hit.session_end)} |{" "}
              {classDays} @ {hit.start_time.slice(0, 5)}-
              {hit.end_time.slice(0, 5)}
            </>
          )}
        </h2>
        <h3 className="text-base font-medium text-neutral-600 dark:text-neutral-300">
          {"Section "} <Highlight attribute="section" hit={hit} /> |{" "}
          <Highlight attribute="instructors" hit={hit} /> |{" "}
          <Highlight attribute="grading" hit={hit} /> |{" "}
          <Highlight attribute="instruction_mode" hit={hit} /> |{" "}
          {!!hit.course_location && (
            <>
              {"Location "}
              <Highlight attribute="course_location" hit={hit} /> |{" "}
            </>
          )}
          <Highlight attribute="component" hit={hit} />
        </h3>
        {!!hit.description && (
          <p className="text-sm text-neutral-600 dark:text-neutral-400">
            <Highlight attribute="description" hit={hit} />
          </p>
        )}
      </div>
    );
  else if (isCancelled) {
    return (
      <div className="space-y-1.5">
        <div className="text-xl font-bold">
          <h1 className="text-red-500 inline">CANCELLED: </h1>
          <h1>
            <Highlight attribute="class_name" hit={hit} /> (
            <Highlight attribute="term" hit={hit} />{" "}
            <Highlight attribute="year" hit={hit} />)
          </h1>{" "}
        </div>
        <h2 className="text-lg font-semibold">
          #
          <Highlight
            className="underline"
            attribute="class_number"
            hit={hit}
          />{" "}
          | <Highlight attribute="school_name" hit={hit} /> | Units:{" "}
          <Highlight attribute="units" hit={hit} />
        </h2>
      </div>
    );
  }
};

export default CourseCard;
