# nyuclassesfrom.xyz

Make NYU course search fast.

If we think Harvard's online catalog is slow... we don't even have a graphQL endpoint at NYU XD.

All points aside, the network is huge, and it's very annoying to wait for a online catalog search.
Furthermore, NYU's status as a global campus with over 40+ locations globally makes it more hard to manage from a catagorical point of view.

Different schools, different classes, and different timezones all blend together to be a slow process.

As such, this project is an attempt to gather fast results for course search, as inspired from classes.wtf, coded in svelte, rust, and redis.

We want to provide a service that will help students easily select and find classes with the given structure at NYU.

## Data Fetching

- catch all the school codes and subject codes
  - ex: SHU = shanghai CSCI = computer science
- go through and fetch each course based off subject & school
  - https://schedge.a1liu.com/2022/fa/SHU/CSCI
- search for course based on full=true, query=courseName, school, subject
  https://schedge.a1liu.com/2022/fa/search?full=true&query={courseName}&school={schoolCode}&subject={subjectCode}
- stitch and format the json

- school codes (fetch) + subject codes (fetch) => list of combos (SHU/CSCI) => courses (fetch) => list of name, courseId, school, subject => search (fetches) => final list of everything per each course
