export function formatSession(start: string, end: string | undefined) {
  const monthNames = [
    "Jan. ",
    "Feb. ",
    "Mar. ",
    "Apr. ",
    "May. ",
    "Jun. ",
    "Jul. ",
    "Aug. ",
    "Sep. ",
    "Oct. ",
    "Nov. ",
    "Dec. ",
  ];

  const startMonth = monthNames[Number(start.slice(5, 7)) - 1];
  const startDay = start.slice(8, 10);
  const endMonth = monthNames[Number(end?.slice(5, 7)) - 1];
  const endDay = end?.slice(8, 10);

  if (typeof end === undefined) return String(startMonth + startDay);
  else if (typeof end !== undefined)
    return String(startMonth + startDay + " - " + endMonth + endDay);
}
