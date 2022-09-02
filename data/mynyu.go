// Public data source for nyu's public course search, a clone for Albert

package datasource

import (
	"bytes"
	"crypto/md5"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strconv"
	"strings"
)

// the official NYU course catalog search endpoint
// as found here: https://anypoint.mulesoft.com/exchange/portals/nyu-0/
const nyuEndpoint = 

// Thanks to Andrew Liu and Nicholas Yang for Sledge, a deprecated (but helpful) API for the course search
// as found here: https://schedge.a1liu.com/