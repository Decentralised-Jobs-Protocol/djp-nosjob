A Beginner's Guide to Nostr's Core Concepts

1. Introduction: Decoding Nostr

Welcome to the world of Nostr! This guide is designed to demystify the protocol's fundamental building blocks in a simple and clear way. We won't get lost in the weeds; instead, we will focus on how simple messages, known as "events," are structured with special labels called "tags" to create a surprisingly simple yet powerful system for building a global, censorship-resistant social network.

2. The Language of Nostr: Understanding Tags

At its heart, Nostr uses tags as the primary method for linking information and people. Think of tags as a special, standardized language that every application on the network can understand. Unlike a traditional platform where a central database links everything together, Nostr achieves this in a decentralized way using only the information contained within the messages themselves. Because these tags have the same meaning everywhere, they allow for a consistent way to create replies, mention users, and reference content across the entire Nostr ecosystem.

Now that you see how tags form a common language for linking data, let's explore the three most essential "words" in that language.

3. The Three Standard Tags: Building Connections

This section will break down the three most common standard tags you'll encounter: e, p, and a. Mastering these is the key to unlocking how Nostr truly works.

3.1 The 'e' Tag: Linking to Other Events

The e tag's primary purpose is to refer to another event. This is the core mechanism for creating replies to a message, quoting another user's post, or making any kind of reference to a previous message on the network. This simple tag is the key to allowing threaded conversations, making Nostr feel as intuitive as any familiar social network.

Component	Description	Status
e	The tag identifier.	Required
<32-bytes lowercase hex of the id of another event>	The unique ID of the event being referenced.	Required
<recommended relay URL>	An optional hint for clients, suggesting a relay where the referenced event is likely to be found.	Optional
<32-bytes lowercase hex of the author's pubkey>	The public key of the original event's author, allowing for faster attribution.	Optional

3.2 The 'p' Tag: Referring to People

The p tag is used to refer to another user by their unique public key (pubkey). It's how you specifically point to another person's profile within an event. This is how you "mention" or "tag" other users in a post, ensuring they can be notified and making rich social interaction possible.

Component	Description	Status
p	The tag identifier.	Required
<32-bytes lowercase hex of a pubkey>	The public key of the user being referenced.	Required
<recommended relay URL>	An optional hint for clients, suggesting a relay where the user's profile and other events can be found.	Optional

3.3 The 'a' Tag: Pinpointing Specific Content

The a tag is a more specialized tool for referencing events that can be changed or updated over time, known as "addressable" or "replaceable" events. Think of this tag as the way to link to something permanent that can still be updated, like a user's profile information or a living blog post.

The a tag comes in two primary forms:

* For Addressable Events This form is used to point to a specific, named piece of content created by a user, which is identified by a d tag. The d tag value acts as a unique, human-readable identifier for the content, like a slug in a blog post URL (e.g., 'my-first-post').
* For Normal Replaceable Events This form is used for more general replaceable events that don't have a specific identifier beyond their kind and author, such as a user's profile.
* Note the important trailing colon. This signifies that you're pointing to a general 'kind' of event from that user (like their current profile) rather than a specific, named piece of content.

Understanding these tags individually is helpful, but their true power is unlocked when you see the network they create together.

4. How These Tags Create a Network

This elegant tagging system is what allows a decentralized network of independent messages to function like a cohesive social platform. By combining these basic tags, Nostr achieves complex interactions without needing a central authority.

* Conversations: The e tag enables any client to independently reconstruct an entire conversation thread by following the chain of references from one event to the next, all without asking a central server for the "correct" order.
* Community: The p tag allows users to build a portable social graph. By embedding references to other people in their own messages, users create connections that are not owned or controlled by any single platform.
* Content: The a tag creates stable, permanent addresses for content that isn't tied to a specific server or domain. This allows for the creation of persistent, updatable resources like profiles and articles that can be referenced from anywhere on the network.

With this foundation in place, you're ready to appreciate the bigger picture.

5. Conclusion: Your First Step into Nostr

You've now seen the engine room of Nostr. By learning how simple events use the e, p, and a tags, you have taken the first and most crucial step toward grasping how the entire protocol functions. This knowledge reveals how a simple, resilient, and decentralized social network is built not on complex servers, but on these elegant and fundamental building blocks.
