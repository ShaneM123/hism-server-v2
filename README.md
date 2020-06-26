
<h1><code>HISM</code></h1>
  <strong>An inventory management system for neighbourhoods during a crisis: <a href="https://github.com/ShaneM123/hism">hism</a>.</strong>
  <p> 
  <strong> This is the backend server and database for Hism </strong>

<p> There are two ways to run this erver, either build it yourself or use the docker container to do the lifting for you</p>
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