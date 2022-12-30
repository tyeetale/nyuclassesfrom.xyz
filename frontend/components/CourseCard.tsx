import { Highlight } from "react-instantsearch-dom";

const CourseCard = ({ hit }: any) => {
  return (
    <div className="space-y-1.5">
      <h1 className="text-xl font-bold">
        <Highlight attribute="class_name" hit={hit} /> (
        <Highlight attribute="term" hit={hit} />{" "}
        <Highlight attribute="year" hit={hit} />)
      </h1>
      <h2 className="text-lg font-semibold">
        #<Highlight attribute="class_number" hit={hit} /> |{" "}
        <Highlight attribute="school_name" hit={hit} /> | Units:{" "}
        <Highlight attribute="units" hit={hit} />
        {!!hit.session_start && (
          <>
            {" "}
            | <Highlight attribute="session_start" hit={hit} />
          </>
        )}
      </h2>
      <h3 className="text-base font-medium">
        <Highlight attribute="section" hit={hit} /> |{" "}
        <Highlight attribute="instructors" hit={hit} /> |{" "}
        <Highlight attribute="grading" hit={hit} /> |{" "}
        <Highlight attribute="instruction_mode" hit={hit} /> |{" "}
        {!!hit.course_location && (
          <>
            <Highlight attribute="course_location" hit={hit} /> |{" "}
          </>
        )}
        <Highlight attribute="component" hit={hit} />
      </h3>
      {!!hit.description && (
        <p className="text-sm">
          <Highlight attribute="description" hit={hit} />
        </p>
      )}
    </div>
  );
};

export default CourseCard;
