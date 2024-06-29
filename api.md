# Broodkruimel API reference


### Users
- `POST`: `/api/users`:  
   Creates a user on the server and returns the UUIDv4 of the newly created user.
- `GET`: `/api/users`:  
   Returns a list of all users on the server.

### Location
- `PUT`: `/api/users/{uuid}/location`:  
   Adds a location to a user's location data.  
   `{uuid}`: The UUID of the user you want to send the location for.  
   **Returns**:  
   TODO!

### Others
- `GET`: `/ping`:  
   Straightforward