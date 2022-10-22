# nyuclassesfrom.xyz

Make NYU course search fast.

If we think Harvard's online catalog is slow... we don't even have a graphQL endpoint at NYU XD.

All jokes aside, our course search is huge, and it's very annoying to wait for a online catalog search. Furthermore, NYU's status as a global campus with over 40+ locations globally makes it harder to manage from a catagorical point of view, compared to other course searches.Different schools, different classes, and different timezones all blend together, which is why the current website is a slow process (Not to mention highly cryptic and bundled weirdly).

As such, this project is an attempt to gather fast results for course search, as inspired from classes.wtf.

Please keep in mind this is a hobby project between friends, and in no ways can we gaurantee that everything will be working 100% of the time. We do not hold any responsibility towards registering for courses, or any implications to using this service.

With all this being said, it is our aspiration to help fellow NYU students easily find classes to their upcoming semesters, and we hope this helps!

## Data Fetching

- catch all the school codes and subject codes
  - ex: SHU = shanghai CSCI = computer science
- go through and fetch each course based off subject & school
  - https://schedge.a1liu.com/2022/fa/SHU/CSCI
- search for course based on full=true, query=courseName, school, subject
  https://schedge.a1liu.com/2022/fa/search?full=true&query={courseName}&school={schoolCode}&subject={subjectCode}
- stitch and format the json

- school codes (fetch) + subject codes (fetch) => list of combos (SHU/CSCI) => courses (fetch) => list of name, courseId, school, subject => search (fetches) => final list of everything per each course

# Running the Frontend

```
cd frontend
npm install
npm run dev
```

# TODOs

- [] Add Wrapping Formatting for frontend
- [] Finish Scraping
- [] Deploy Redis Server
- [] Design database structure
