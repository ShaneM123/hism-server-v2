
<h1><code>HISM</code></h1>
  <strong>An Inventory management system for neighbourhoods during a crisis: <a href="https://github.com/ShaneM123/hism">hism</a>.</strong>
  <p> <p>Hism is a way to connect people with their neighbours to allow them to trade items, ie a hammer for a screwdriver. Users can see their neighbours inventory and what they have in excess and request to trade or borrow items at will. Hism will use location tracking to ensure the user is trading with nearby neighbours</p>
  <strong> This is the backend server and database for <a href="https://github.com/ShaneM123/hism">Hism</a> </strong>
<p>! Hism is still in development. location tracking and frontend design are still in progress !</p>
<p> There are two ways to run this server, either build it yourself or use the docker container to do the lifting for you </p>
<h2> Via Docker: </h2>
<p>Make sure Docker is installed. Then go to the project folder and type the follow command: </p>
<p><code>docker build . --tag hism:v01</code>  </p>
<p> sudo privileges may be required if you are using linux </p>
<p>once the container is built (this may take 10 minutes on the first attempt) type the following command: </p>
<p><code>docker run -p  8443:8443 hism:v01 </code></p>
<p> Your Hism server should now be running from a docker container, congratulations! to stop the container use the command: </p>
<p><code>docker stop "Container Name" </code> </p>
<p> to get the container name (id) type: </p>
<p><code>docker ps -a</code></p>

<p>below is the manual build if you're feeling adventurous</p> 
<h2>Manual build:</h2>
<p>firstly, you will need redis server installed for session caching, sqlite (and its library file) for the backend. Along with Rust and Diesel ORM for rust framework</p>

<h2> possible API Requests </h2>
<p> until the frontend is complete, API requests are the best way to communicate with the backend currently.</p>

<p>firstly, you can create a user with the following curl request:</p>
<p>curl --insecure -H 'Content-Type: application/json' -X POST https://0.0.0.0:8443/createusers -d '{"username":"Tommy","password":"pass123"}'
</p>
<p>once a user is created, you can login:</p>
<p>curl --insecure -c mycookie.txt  -H 'Content-Type: application/json' -X POST https://0.0.0.0:8443/login -d '{"username":"Tommy","password":"pass123"}'
</p>

<p>a quick text to check that youre still logged in: </p>
<p>curl --insecure -b mycookie.txt -H 'Content-Type: application/json' -X GET https://0.0.0.0:8443/whoami
</p>

<p>Finally, to log out:</p>
<p>curl --insecure -b mycookie.txt -H 'Content-Type: application/json' -X POST https://0.0.0.0:8443/logout</p>


<p>there are many other possible requests, that I won't list here, but feel free to look at the 'routes' in the source code to see what requests are possible </p>
