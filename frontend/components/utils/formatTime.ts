export function formatTime(time: string) {
  const isPM = Number(time.slice(0, 2)) > 12;
  const pmHr = Number(time.slice(0, 2)) - 12;

  if (isPM) return String(pmHr + time.slice(2, 5) + " PM");
  else if (!isPM) return String(time.slice(0, 5) + " AM");
}
