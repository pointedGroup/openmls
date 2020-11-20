initSidebarItems({"enum":[["HandshakeMessageFormat",""],["InvalidMessageError",""],["MLSMessage","Unified message type"],["ManagedGroupError",""]],"struct":[["ManagedGroup","A `ManagedGroup` represents an `MLSGroup` with an easier, high-level API designed to be used in production. The API exposes high level functions to manage a group by adding/removing members, get the current member list, etc."],["ManagedGroupCallbacks","Collection of callback functions that are passed to a `ManagedGroup` as part of the configurations Callback functions are optional. If no validator function is specified for a certain proposal type, any semantically valid proposal will be accepted. Validator fucntions returan a `bool`, depending on whether the proposal is accepted by the application policy."],["ManagedGroupConfig","Specifies the configuration parameters for a managed group"],["UpdatePolicy","Specifies in which intervals the own leaf node should be updated"]],"type":[["AppMessageReceived","Event listener function for application messages `(managed_group: &ManagedGroup, aad: &[u8], sender: &Sender, message: &[u8])`"],["ErrorOccured","Event listener function for errors that occur `(managed_group: &ManagedGroup, error: ManagedGroupError)`"],["InvalidMessageReceived","Event listener function for invalid messages `(managed_group: &ManagedGroup, aad_option: Option<&[u8]>, sender_option: Option<&Sender>, error: InvalidMessageError)`"],["MemberAdded","Event listener function for AddProposals `(managed_group: &ManagedGroup, aad: &[u8], sender: &Sender, add_proposal: &AddProposal)`"],["MemberRemoved","Event listener function for RemoveProposals `(managed_group: &ManagedGroup, aad: &[u8], sender: &Sender, remove_proposal: &RemoveProposal)`"],["MemberUpdated","Event listener function for UpdateProposals `(managed_group: &ManagedGroup, aad: &[u8], sender: &Sender, update_proposal: &UpdateProposal)`"],["ValidateAdd","Validator function for AddProposals `(managed_group: &ManagedGroup, aad: &[u8], sender: &Sender, aad_porposal: &AddProposal) -> bool`"],["ValidateRemove","Validator function for RemoveProposals `(managed_group: &ManagedGroup, aad: &[u8], sender: &Sender, remove_porposal: &RemoveProposal) -> bool`"]]});