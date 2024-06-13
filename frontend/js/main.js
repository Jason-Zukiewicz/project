// Wait until the DOM is fully loaded
document.addEventListener( "DOMContentLoaded", function () {
    // Create a new div element
    let newDiv = document.createElement( "div" );

    // Add some content to the div
    newDiv.innerHTML = "<h2>Welcome to the Untitled Website!</h2><p>Enjoy our collection of vocabulary games.</p>";

    // Style the div (optional)
    newDiv.style.border = "2px solid black";
    newDiv.style.padding = "10px";
    newDiv.style.margin = "10px";
    newDiv.style.backgroundColor = "#f0f0f0";

    // Append the new div to the body
    document.body.appendChild( newDiv );
} );
